const url = process.env.EXPO_PUBLIC_API_URL;

if (!url) {
  throw new Error(
    "EXPO_PUBLIC_API_URL is not set. Copy mobile/.env.example to mobile/.env.local, " +
      "set your PC's address, and restart Expo with: npx expo start --clear",
  );
}

export const API_URL = url.replace(/\/+$/, "");
