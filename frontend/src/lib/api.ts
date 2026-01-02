import axios from "axios";

export const api = axios.create({
  baseURL: "http://localhost:3000",
});

const DEV_USER_ID = "79b111bb-e602-490b-ad0b-a86d8331ff5f";

api.interceptors.request.use((config) => {
  if (DEV_USER_ID) {
    config.headers["x-user-id"] = DEV_USER_ID;
  }
  return config;
});
