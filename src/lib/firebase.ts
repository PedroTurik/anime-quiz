import { initializeApp } from "firebase/app";
import { getDatabase, ref, onDisconnect } from "firebase/database";
import { getAuth, onAuthStateChanged, type User } from "firebase/auth";
import { writable, type Readable, derived } from "svelte/store";

const firebaseConfig = {
  apiKey: "AIzaSyAtZbHWZfewFL8mAOLlnAnOPgh6xELXBfc",
  authDomain: "animequiz-aa811.firebaseapp.com",
  databaseURL: "https://animequiz-aa811-default-rtdb.firebaseio.com",
  projectId: "animequiz-aa811",
  storageBucket: "animequiz-aa811.appspot.com",
  messagingSenderId: "974012375043",
  appId: "1:974012375043:web:dc9c088e96d702dfb66e36"
};

// Initialize Firebase
export const app = initializeApp(firebaseConfig);
export const db = getDatabase();
export const auth = getAuth();

export const user = writable<User | null>(null);

onAuthStateChanged(auth, (newUser) => {
  user.set(newUser);
});