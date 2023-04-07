import ora from "ora";
import path from "path";
import { PKG_ROOT } from "./const.js";
import inquirer from "inquirer";
import fs from "fs-extra";
import chalk from "chalk";
import { execa } from "execa";
import { getCurrentPackageManager } from "./utils.js";

export const installProject = async (projectName: string) => {
  const spinner = ora("Setting up your project...").start();

  const projectDir = path.resolve(process.cwd(), projectName);
  const srcDir = path.join(PKG_ROOT, "template");

  if (fs.existsSync(projectDir)) {
    if (fs.readdirSync(projectDir).length === 0) {
      if (projectName !== ".")
        spinner.info(
          `${chalk.cyan.bold(projectName)} exists but is empty, continuing...\n`
        );
    } else {
      spinner.stopAndPersist();
      const { overwriteDir } = await inquirer.prompt<{
        overwriteDir: "abort" | "clear" | "overwrite";
      }>({
        name: "overwriteDir",
        type: "list",
        message: `${chalk.redBright.bold("Warning:")} ${chalk.cyan.bold(
          projectName
        )} already exists and isn't empty. How would you like to proceed?`,
        choices: [
          {
            name: "Abort installation (recommended)",
            value: "abort",
            short: "Abort",
          },
          {
            name: "Clear the directory and continue installation",
            value: "clear",
            short: "Clear",
          },
          {
            name: "Continue installation and overwrite conflicting files",
            value: "overwrite",
            short: "Overwrite",
          },
        ],
        default: "abort",
      });
      if (overwriteDir === "abort") {
        spinner.fail("Aborting installation...");
        process.exit(1);
      }

      const overwriteAction =
        overwriteDir === "clear"
          ? "clear the directory"
          : "overwrite conflicting files";

      const { confirmOverwriteDir } = await inquirer.prompt<{
        confirmOverwriteDir: boolean;
      }>({
        name: "confirmOverwriteDir",
        type: "confirm",
        message: `Are you sure you want to ${overwriteAction}?`,
        default: false,
      });

      if (!confirmOverwriteDir) {
        spinner.fail("Aborting installation...");
        process.exit(1);
      }

      if (overwriteDir === "clear") {
        spinner.info(
          `Emptying ${chalk.cyan.bold(projectName)} and creating t3 app..\n`
        );
        fs.emptyDirSync(projectDir);
      }
    }
  }

  spinner.start();

  fs.copySync(srcDir, projectDir);

  spinner.text = "Installing dependencies...";
  const packageManager = getCurrentPackageManager();
  await execa(packageManager, packageManager === "yarn" ? [] : ["install"], {
    cwd: projectDir,
  });

  const scaffoldedName =
    projectName === "." ? path.basename(process.cwd()) : projectName;

  const packageJsonPath = path.join(projectDir, "package.json");
  const packageJson = fs.readJsonSync(packageJsonPath);
  packageJson.name = scaffoldedName;
  fs.writeJsonSync(packageJsonPath, packageJson, { spaces: 2 });

  spinner.succeed(
    `${chalk.cyan.bold(scaffoldedName)} ${chalk.green(
      "scaffolded successfully!"
    )}\n`
  );
};
