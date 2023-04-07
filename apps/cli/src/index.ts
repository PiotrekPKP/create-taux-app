#! /usr/bin/env node

import inquirer from "inquirer";
import { displayTitle, validateAppName } from "./utils.js";
import ora from "ora";

const main = async () => {
  displayTitle();

  const { projectName } = await inquirer.prompt<{ projectName: string }>({
    message: "How is your project called?",
    name: "projectName",
    transformer: (input: string) => input.trim(),
    validate: validateAppName,
  });

  console.log();

  const spinner = ora("Setting up your project...").start();
};

await main();
