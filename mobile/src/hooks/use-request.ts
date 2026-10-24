import { useFocusEffect } from "expo-router";
import { useCallback, useRef, useState } from "react";

import { describeError } from "@/api/errors";

type State<T> = { data: T | null; error: string | null; loading: boolean };

export type Request<T> = State<T> & {
  reload: () => Promise<void>;
};

export function useRequest<T>(fetcher: () => Promise<T>): Request<T> {
  const [state, setState] = useState<State<T>>({
    data: null,
    error: null,
    loading: true,
  });

  const latestFetcher = useRef(fetcher);
  latestFetcher.current = fetcher;

  const requestId = useRef(0);

  const reload = useCallback(async () => {
    const id = ++requestId.current;
    setState((current) => ({ ...current, loading: true, error: null }));

    try {
      const data = await latestFetcher.current();
      if (id === requestId.current) {
        setState({ data, error: null, loading: false });
      }
    } catch (error) {
      if (id === requestId.current) {
        setState((current) => ({
          data: current.data,
          error: describeError(error),
          loading: false,
        }));
      }
    }
  }, []);

  useFocusEffect(
    useCallback(() => {
      void reload();

      return () => {
        requestId.current += 1;
      };
    }, [reload]),
  );

  return { ...state, reload };
}
