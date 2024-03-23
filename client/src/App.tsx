import * as React from "react";
import "./App.css";

function App() {
  let [messages, setMessages] = React.useState<Array<string>>([]);
  let textBox: React.MutableRefObject<null | HTMLInputElement> =
    React.useRef(null);

  function append_message(msg: string) {
    messages.push(msg);
    setMessages([...messages]);
  }

  React.useEffect(() => {
    let message_stream = new WebSocket("ws://localhost:8080/connect/defRoom");
    message_stream.onmessage = (event) => {
      console.log(event.data);
      append_message(event.data);
      console.log(messages);
    };
  }, []);

  async function sendMessage(message: string) {
    await fetch("http://localhost:8080/connect/defRoom", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Allow-Origin": "*",
      },
      body: JSON.stringify(message),
    }).then((ret) => {
      console.log(ret.ok ? "Message sent ok" : "error sending message");
    });
  }

  function submit(key: React.KeyboardEvent) {
    if (key.key === "Enter" && textBox.current != null) {
      sendMessage(textBox.current.value);
      textBox.current.value = "";
    }
  }

  return (
    <div className="app">
      <div className="message-view">
        {messages.map((msg, i) => {
          return <div className="message" key={i}>{msg}</div>;
        }).reverse()}
      </div>
      <div className="footer">
        <input className="text-input" ref={textBox} onKeyDown={submit}></input>
      </div>
    </div>
  );
}

export default App;
