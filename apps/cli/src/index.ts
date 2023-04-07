#! /usr/bin/env node

import inquirer from "inquirer";
import { displayTitle, validateAppName } from "./utils.js";
import { installProject } from "./installer.js";

const main = async () => {
  displayTitle();

  const { projectName } = await inquirer.prompt<{ projectName: string }>({
    message: "How is your project called?",
    name: "projectName",
    transformer: (input: string) => input.trim(),
    validate: validateAppName,
  });

  console.log();

  await installProject(projectName);
};

await main();
