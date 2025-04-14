import FrameView from "~/components/frame";
import type { Route } from "../+types/root";

export function meta({ }: Route.MetaArgs) {
  return [
    { title: "Lambda Nodes" },
    { name: "description", content: "Welcome to React Router!" },
  ];
}

export default function Home() {
  return (
    <FrameView title="Home" subtitle="Welcome to the home page">
      <h1>Hello</h1>
    </FrameView>
  );
}
