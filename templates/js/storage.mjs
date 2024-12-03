const staticContent = [
  { id: "1", text: "text 1", parent: 0 },
  { id: "2", text: "text 2", parent: 1 },
  { id: "3", text: "text 3", parent: 2 },
];

const storagePut = () => {
  console.log("placing", staticContent);
  localStorage.setItem("task-fortress-data", JSON.stringify(staticContent));
};

const storageGet = () => {
  const items = localStorage.getItem("task-fortress-data");
  console.log("read", JSON.parse(items));
};

// internal API
export { storagePut };

// external API
window.storageGet = storageGet;
