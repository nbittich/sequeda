const execute = async (db) => {
    const myCollection = await db.collection("myCollection");
    await myCollection.insertOne({
      name: "kikooLol",
      age: 34,
    });
  };
  const rollback = async (db) => console.log("rollbacked!");
  module.exports = {
    targetDatabases: ['demo'],
    description: 'another one with a description and reduced to a single db',
    rollback,
    execute,
  };
  