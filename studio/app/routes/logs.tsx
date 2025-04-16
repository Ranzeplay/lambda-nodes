import { useEffect, useState } from "react";
import FrameView from "~/components/frame";
import { TableHeader, TableRow, TableHead, TableBody, TableCell, Table } from "~/components/ui/table";
import { serverAddress } from "~/lib/env";

type Log = {
	id: number;
	level: "info" | "warn" | "error";
	category: string;
	message: string;
	createAt: string;
};

export default function LogsPage() {
	const [logs, setLogs] = useState<Log[]>([]);
	useEffect(() => {
		fetch(serverAddress + "/api/logs")
			.then((response) => response.json())
			.then((data) => setLogs(data))
			.catch((error) => console.error("Error fetching logs:", error));
	}, []);

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
						{logs.map((log) => (
							<TableRow key={log.id}>
								<TableCell className="text-xs">{new Date(log.createAt).toLocaleString()}</TableCell>
								<TableCell className={`text-xs ${log.category == "info" && "text-blue-500"} ${log.category == "warn" && "text-yellow-600"} ${log.category == "error" && "text-red-500"}`}>{log.level}</TableCell>
								<TableCell className="text-xs">{log.category}</TableCell>
								<TableCell className="text-xs">{log.message}</TableCell>
							</TableRow>
						))}
					</TableBody>
				</Table>
			</div>
		</FrameView>
	)
}
