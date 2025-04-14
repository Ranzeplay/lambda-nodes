import FrameView from "~/components/frame";
import { TableHeader, TableRow, TableHead, TableBody, TableCell, Table } from "~/components/ui/table";

export default function LogsPage() {
	return (
		<FrameView title="Logs" subtitle="View logs">
			<div className="flex flex-col space-y-4">
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead className="font-bold w-[120px]">Timestamp</TableHead>
							<TableHead className="font-bold">Level</TableHead>
							<TableHead className="font-bold">Category</TableHead>
							<TableHead className="font-bold">Description</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						<TableRow>
							<TableCell className="font-medium">{new Date().toUTCString()}</TableCell>
							<TableCell><p className="text-red-500">Error</p></TableCell>
							<TableCell>Runner</TableCell>
							<TableCell>Exception occurred while executing a pipeline</TableCell>
						</TableRow>
					</TableBody>
				</Table>
			</div>
		</FrameView>
	)
}
