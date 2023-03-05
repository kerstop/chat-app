import * as React from 'react'
import './App.css'

function App() {

  let [messages, setMessages] = React.useState<Array<string>>([]);
  let textBox = React.useRef(null);
  //React.useEffect()

  async function sendMessage(message: string) {
    await fetch("http://localhost:8080/connect/defRoom", {
      method: "POST",
      headers: {
        'Content-Type' : 'application/json',
        'Access-Control-Allow-Origin': "*",
      },
      body: JSON.stringify(message)
    }).then((ret) => {
      console.log(ret.ok?"Message sent ok": "error sending message")
    })
  }

  function submit(key:React.KeyboardEvent) {
    if (key.key === "Enter") {
      sendMessage("click")
    }
  }

  return <>
    <input ref={textBox} onKeyDown={submit}></input>
    <div>
      {messages.map((msg) => { return (<div>{msg}</div>)})}
    </div>
  </>
}

export default App
