import { Button } from 'antd';
import style from './index.module.less';
import { invoke } from "@tauri-apps/api/core";
import { useState } from 'react';

export default function Welcome() {
  const [greetMsg, setGreetMsg] = useState("");
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name:"qwikpage" }));
  }


  return (
    <div className={style.welcome} id="welcome">
      <section className={style.footer}>
        <h1>欢迎使用 QwikPage 搭建平台</h1>
          <Button type="primary" size="large" onClick={greet}>
            快速开始
          </Button>
        <p>{greetMsg}</p>
      </section>
    </div>
  );
}
