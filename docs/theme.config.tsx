import Head from "next/head";
import Image from "next/image";
import { DocsThemeConfig } from "nextra-theme-docs";

const config: DocsThemeConfig = {
  logo: (
    <span style={{ display: "flex", alignItems: "center", gap: "13px" }}>
      <Image alt="" src="/logo.svg" width={30} height={30} />
      <span>Create Taux App</span>
    </span>
  ),
  project: {
    link: "https://github.com/PiotrekPKP/create-taux-app",
  },
  docsRepositoryBase:
    "https://github.com/PiotrekPKP/create-taux-app/tree/main/apps/docs",
  footer: {
    text: "Create Taux App",
  },
  useNextSeoProps: () => ({
    titleTemplate: "%s - Create Taux App",
  }),
};

export default config;
