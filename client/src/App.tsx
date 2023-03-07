import * as React from 'react'
import './App.css'

function App() {

  let [messages, setMessages] = React.useState<Array<string>>([]);
  let textBox: React.MutableRefObject<null | HTMLInputElement> = React.useRef(null);

  React.useEffect(() => {
    let message_stream = new EventSource("http://localhost:8080/connect/defRoom")
    message_stream.onmessage = (event) => {
      console.log(event.data);
      setMessages([...messages, event.data])
      console.log(messages)
    }
  }, [])

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
    if (key.key === "Enter" && textBox.current != null) {
      sendMessage(textBox.current.value)
      textBox.current.value = "";
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
