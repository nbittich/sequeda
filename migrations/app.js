const fs = require("fs").promises;
const path = require("path");
const mongodb = require("mongodb");
const { MongoClient } = mongodb;

const MONGO_HOST = envOrThrow("MONGO_HOST");
const MONGO_PORT = envOrThrow("MONGO_PORT");
const MONGO_USERNAME = envOrThrow("MONGO_USERNAME");
const MONGO_PASSWORD = envOrThrow("MONGO_PASSWORD");

const MIGRATIONS_DIR = process.env.MIGRATIONS_DIR || "/migrations";
const MIGRATIONS_DB = process.env.MIGRATIONS_DB || "_migration";

const INTERNAL_DATABASES = ["admin", "config", "local", MIGRATIONS_DB];

const CLIENT = new MongoClient(
  `mongodb://${MONGO_USERNAME}:${MONGO_PASSWORD}@${MONGO_HOST}:${MONGO_PORT}`
);

const ADMIN_CLIENT = CLIENT.db().admin();

const CONTEXT = {
  uuid: require('uuid').v4,
  moment: require('moment'),
  now: () => new Date().toISOString().slice(0, 23)
}

async function main() {
  await ping();
  const fileNames = await readDirRecursive(MIGRATIONS_DIR);
  const allowedDatabases = (await ADMIN_CLIENT.listDatabases()).databases
    .map((db) => db.name)
    .filter((db) => !INTERNAL_DATABASES.includes(db));
  for (const { file } of fileNames) {
    console.log(`Current file: ${file}`);
    const { execute, rollback, targetDatabases, description } = require(file);
    const databases =
      (targetDatabases || allowedDatabases).filter((db) => allowedDatabases.includes(db)) ||
      allowedDatabases;
    for (const database of databases) {
      const db = CLIENT.db(database);
      const migrationCollection = db.collection(MIGRATIONS_DB);
      const migration = await migrationCollection.findOne({ name: file });
      if (!migration) {
        console.log(
          `Migration ${file} didn't run yet for db ${database}, executing...`
        );
        try {
          await execute(db, CONTEXT);
          const migration = { name: file, executedAt: new Date(), description };
          await migrationCollection.insertOne(migration);
          console.log(`Migration ${file} for db ${database} ran successfully!`);
        } catch (e) {
          console.log(
            `an error occurred while proceeding ${file} for db ${db}. Error: ${e}, try to rollback...`
          );
          try {
            await rollback(db, CONTEXT);
          } catch (e) {
            console.log(`could not rollback ${file} & db ${db}.`);
          }
          process.exit(1);
        }
      }
    }
  }
  console.log('Done!');
}

async function ping() {
  // ping db until available
  let connected = false;
  while (!connected) {
    try {
      console.log("ping database...");
      await ADMIN_CLIENT.ping();
      connected = true;
    } catch (e) {}
  }
  console.log("Connected!");
}

function envOrThrow(key) {
  const value = process.env[key];
  if (!value) {
    const errorMessage = `missing environment variable ${key}`;
    throw Error(errorMessage);
  }
  return value;
}

async function readDirRecursive(dir, filterExtensions = ["js"]) {
  const files = await fs.readdir(dir);

  const resultFiles = [];

  for (const file of files) {
    const filePath = path.join(dir, file);
    const stat = await fs.stat(filePath);
    if (stat.isDirectory()) {
      resultFiles.push(...(await readDirRecursive(`${dir}/${file}`)));
    } else if (filterExtensions.some((ext) => filePath.endsWith(ext))) {
      resultFiles.push({
        time: stat.atimeMs,
        file: filePath,
      });
    } else {
      console.log(`filter file ${filePath}`);
    }
  }

  return resultFiles.sort((a, b) => a.time - b.time);
}

main().then(() => process.exit(0));
