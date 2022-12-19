const execute = async (db, context = {}) => {
  const myCollection = await db.collection("myCollection");
  await myCollection.insertOne({
    name: "kikoo",
    age: 33,
  });
};
const rollback = async (db, context = {}) => console.log("hello world rollbacked!");
module.exports = {
  rollback,
  execute,
};
