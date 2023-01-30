import Head from "next/head";
import { Inter } from "@next/font/google";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
  const [txt, setTxt] = useState<string>("");

  const [country, setCountry] = useState<string>("");

  function operand() {
    invoke("add", { op1: 4, op2: 2 })
      .then((result) => {
        console.log(result);
      })
      .catch(console.error);

    invoke("sub", { op1: 4, op2: 2 })
      .then((result) => {
        console.log(result);
      })
      .catch(console.error);

    invoke("div", { op1: 4, op2: 2 })
      .then((result) => {
        console.log(result);
      })
      .catch(console.error);

    invoke("mul", { op1: 4, op2: 2 })
      .then((result) => {
        console.log(result);
      })
      .catch(console.error);
  }

  function dbConnect(name: string, country: string) {
    invoke("simple_insert", { name, country });
  }

  // operand();

  dbConnect();

  useEffect(() => {
    invoke("greet", { name: "World" }).then(console.log).catch(console.error);
  }, []);

  return (
    <>
      <div className="bg-red-300">
        <h1 className="text-3xl font-bold underline"> Hello World!</h1>
      </div>
      <input
        className="bg-blue-300"
        onChange={(event) => setTxt(event.target.value)}
      ></input>
      <button onClick={() => console.log(txt)}>asdf</button>
      <hr></hr>
      <input
        className="bg-blue-300"
        onChange={(event) => setTxt(event.target.value)}
      ></input>
      <button onClick={() => console.log(txt)}>asdf</button>
      <hr></hr>
      <button onClick={() => dbConnect(txt, country)}>asdf</button>
    </>
  );
}
