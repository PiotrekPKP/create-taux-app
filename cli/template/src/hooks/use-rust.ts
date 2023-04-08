// This file contains two hooks: `useRustMutation` and `useRustQuery`
// `useRustMutation` is used for mutating data, like adding a todo item
// `useRustQuery` is used for querying data, like getting all todo items
// You probably don't want to modify this file

import { RustFunction } from "@/types/rust";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

export type RustFunctionName = keyof RustFunction;

export const useRustCommand = <T extends keyof RustFunction>(fnName: T) => {
  const [data, setData] = useState<RustFunction[T]["return"] | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isError, setIsError] = useState(false);
  const [error, setError] = useState<RustFunction[T]["error"] | null>(null);

  const mutate = async (
    ...args: RustFunction[T]["args"] extends never
      ? [undefined?]
      : [RustFunction[T]["args"]]
  ) => {
    setIsLoading(true);
    setIsError(false);
    setData(null);
    setError(null);

    try {
      const response = await invoke<RustFunction[T]["return"]>(fnName, args[0]);
      setData(response);
      setIsLoading(false);
      return response;
    } catch (error) {
      setError(error as RustFunction[T]["error"]);
      setIsError(true);
      setIsLoading(false);
      throw error;
    }
  };

  return { mutate, data, isLoading, isError, error };
};

export const useRustQuery = <T extends keyof RustFunction>(
  fnName: T,
  ...args: RustFunction[T]["args"] extends never
    ? [undefined?]
    : [RustFunction[T]["args"]]
) => {
  const [data, setData] = useState<RustFunction[T]["return"] | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isError, setIsError] = useState(false);
  const [error, setError] = useState<RustFunction[T]["error"] | null>(null);

  const fetchData = async () => {
    try {
      const data = await invoke<RustFunction[T]["return"]>(fnName, args[0]);
      setData(data);
    } catch (error) {
      setError(error as RustFunction[T]["error"]);
      setIsError(true);
    }

    setIsLoading(false);
  };

  useEffect(() => {
    setIsLoading(true);
    setIsError(false);
    setData(null);
    setError(null);

    fetchData();
  }, []);

  return { data, isLoading, isError, error, refetch: fetchData };
};
