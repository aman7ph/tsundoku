import axios from "axios";

export function describeError(error: unknown): string {
  if (axios.isAxiosError(error)) {
    if (!error.response) {
      return "Could not reach the backend. Is it running, and is the phone on the same Wi-Fi?";
    }

    const body = error.response.data as { error?: string } | undefined;
    return (
      body?.error ??
      `The backend answered with an error (${error.response.status}).`
    );
  }

  return "Something went wrong.";
}
