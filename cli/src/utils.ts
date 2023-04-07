import gradientString from "gradient-string";
import { TITLE_TEXT } from "./const.js";

export type PackageManager = "npm" | "pnpm" | "yarn";

export const getCurrentPackageManager: () => PackageManager = () => {
  const userAgent = process.env.npm_config_user_agent;

  if (userAgent) {
    if (userAgent.startsWith("yarn")) {
      return "yarn";
    } else if (userAgent.startsWith("pnpm")) {
      return "pnpm";
    } else {
      return "npm";
    }
  } else {
    return "npm";
  }
};

const validationRegExp =
  /^(?:@[a-z0-9-*~][a-z0-9-*._~]*\/)?[a-z0-9-~][a-z0-9-._~]*$/;

export const validateAppName = (input: string) => {
  const paths = input.split("/");

  const indexOfDelimiter = paths.findIndex((p) => p.startsWith("@"));

  let appName = paths[paths.length - 1];
  if (paths.findIndex((p) => p.startsWith("@")) !== -1) {
    appName = paths.slice(indexOfDelimiter).join("/");
  }

  if (input === "." || validationRegExp.test(appName ?? "")) {
    return true;
  } else {
    return "App name must consist of only lowercase alphanumeric characters, '-', and '_'";
  }
};

export const displayTitle = () => {
  console.log(gradientString(["#FF006B", "#0085FF"]).multiline(TITLE_TEXT));
};
