const execute = async (db) => {
  const myCollection = await db.collection("myCollection");
  await myCollection.insertOne({
    name: "kikoo",
    age: 33,
  });
};
const rollback = async (db) => console.log("hello world rollbacked!");
module.exports = {
  rollback,
  execute,
};
