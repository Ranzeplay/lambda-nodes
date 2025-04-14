import { Plus } from "lucide-react";
import { Link } from "react-router";
import FrameView from "~/components/frame";
import { Button } from "~/components/ui/button";

export default function LibraryPage() {
	return (
		<FrameView title="Library" subtitle="All scripts">
			<div className="flex flex-row-reverse">
				<Button>
					<Link to="/node/new" className="flex items-center gap-x-2 m-0 p-0">
					<Plus /> New
					</Link>
				</Button>
			</div>
		</FrameView>
	);
}
