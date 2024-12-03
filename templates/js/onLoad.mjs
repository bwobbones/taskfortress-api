import { storagePut } from "./storage.mjs";

const onLoad = () => {
  storagePut();
};

export { onLoad };
