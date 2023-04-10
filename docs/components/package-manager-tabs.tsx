import { Tab, Tabs } from "nextra-theme-docs";

const PKG_MANAGERS = ["pnpm", "npm"];

const PackageManagerTabs: React.FC<React.PropsWithChildren> = ({
  children,
}) => {
  return (
    <Tabs
      items={PKG_MANAGERS}
      onChange={
        (
          value
        ) => {} /* TODO: Set PKG_MANAGERS[value] in `usePkgManager` hook */
      }
    >
      {children}
    </Tabs>
  );
};

export { Tab };

export default PackageManagerTabs;
