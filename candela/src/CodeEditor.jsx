import Editor, { useMonaco } from "@monaco-editor/react";
import { useRef, useEffect } from "react";
import { setUp } from "./editor_setup"
import { invoke } from '@tauri-apps/api/tauri'

const example_code = `1..5 {
  intensity: 100
}`;

export function CodeEditor(props) {
  const editorRef = useRef(null);
  const monaco = useMonaco();

  // TODO: Move this configuration to another file
  // use as guide: https://ohdarling88.medium.com/4-steps-to-add-custom-language-support-to-monaco-editor-5075eafa156d
  useEffect(() => {
    console.log(monaco);
    if (monaco) {    
      setUp(monaco);
      onTextChange(example_code);
    }
  }, [monaco])

  function handleEditorDidMount(editor, _monaco) {
    editorRef.current = editor;
  }

  function handleChange(value, _event) {
    onTextChange(value);
  }

  function onTextChange(value) {
    invoke("on_text_change", { source: value }).then((msg) => props.setConsoleText(msg));
  }

  return (
    <>
      <Editor
        defaultLanguage="lux"
        defaultValue={example_code}
        options={{
          "fontSize": "16"
        }}
        onMount={handleEditorDidMount} 
        onChange={handleChange} />
    </>
  )
}
