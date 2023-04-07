import "@/styles/globals.css";
import type { AppProps } from "next/app";
import { Inter as FontSans } from "next/font/google";

const font = FontSans({
  variable: "--font-sans",
  subsets: ["latin", "latin-ext"],
});

const App = ({ Component, pageProps }: AppProps) => {
  return (
    <>
      <style jsx global>
        {`
          :root {
            --font-sans: ${font.style.fontFamily};
          }
        `}
      </style>

      <div className={`${font.className} ${font.variable}`}>
        <Component {...pageProps} />
      </div>
    </>
  );
};

export default App;
