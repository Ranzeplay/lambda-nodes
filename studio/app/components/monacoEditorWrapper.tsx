import { Editor, type EditorProps } from "@monaco-editor/react";

export default function MonacoEditorWrapper(props: EditorProps) {
	return <Editor {...props} />;
}
