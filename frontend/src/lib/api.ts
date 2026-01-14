import axios from "axios";

export const api = axios.create({
  baseURL: "http://localhost:3000",
  headers: {
    "Content-Type": "application/json",
  },
});

// リクエストの直前に割り込む処理
api.interceptors.request.use((config) => {
  // local storageにtokenがあればヘッダーにセットする
  const token = localStorage.getItem("token");
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});
