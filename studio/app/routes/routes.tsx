import { useEffect, useState } from "react";
import { toast } from "sonner";
import FrameView from "~/components/frame";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from "~/components/ui/select";
import { TableHeader, TableRow, TableHead, TableBody, Table, TableCell } from "~/components/ui/table";
import { serverAddress } from "~/lib/env";

export default function RoutesPage() {
	const [routes, setRoutes] = useState<any[]>([]);
	useEffect(() => {
		// Fetch routes from the API or any other source
		fetch(serverAddress + "/api/routes")
			.then((response) => response.json())
			.then((data) => setRoutes(data))
			.catch((error) => toast.error("Error fetching routes:", error));
	}, []);

	function deleteRoute(id: string) {
		fetch(serverAddress + "/api/routes/" + id, {
			method: "DELETE",
		})
			.then((response) => {
				if (response.ok) {
					setRoutes(routes.filter((route) => route.id !== id));
					toast.success("Route deleted successfully.");
				} else {
					toast.error("Error deleting route.");
				}
			})
			.catch((error) => toast.error("Error deleting route:", error));
	}

	return (
		<FrameView title="Routes" subtitle="Manage routes">
			<div className="flex flex-col space-y-4">
				<Table>
					<TableHeader>
						<TableRow>
							<TableHead className="font-bold w-[120px]">Id</TableHead>
							<TableHead className="font-bold">Pipeline Id</TableHead>
							<TableHead className="font-bold">Method</TableHead>
							<TableHead className="font-bold">Path</TableHead>
							<TableHead className="font-bold text-right pr-6">Operations</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{routes.map((route) => (
							<TableRow key={route.id}>
								<TableCell className="font-medium">{route.id}</TableCell>
								<TableCell>{route.pipelineId}</TableCell>
								<TableCell>{route.method}</TableCell>
								<TableCell>{route.path}</TableCell>
								<TableCell className="text-right">
									<Button variant="link" className="text-red-500 hover:text-red-700 hover:underline *:cursor-pointer" onClick={() => deleteRoute(route.id)}>
										Delete
									</Button>
								</TableCell>
							</TableRow>
						))}
						<NewRouteComponent />
					</TableBody>
				</Table>
			</div>
		</FrameView>
	);
}

function NewRouteComponent() {
	const [method, setMethod] = useState("");
	const [path, setPath] = useState("");
	const [pipelineId, setPipelineId] = useState("");

	const [availablePipelines, setAvailablePipelines] = useState<any[]>([]);
	useEffect(() => {
		// Fetch pipelines from the API or any other source
		fetch(serverAddress + "/api/pipelines")
			.then((response) => response.json())
			.then((data) => setAvailablePipelines(data))
			.catch((error) => toast.error("Error fetching pipelines:", error));
	}, []);

	async function submit() {
		if (!method || !path || !pipelineId) {
			toast.error("Please fill in all fields.");
			return;
		}

		const response = await fetch(serverAddress + "/api/routes", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				method,
				path,
				pipelineId,
			}),
		});

		if (response.ok) {
			toast.success("Route created successfully.");
			setMethod("");
			setPath("");
			setPipelineId("");
		} else {
			toast.error("Error creating route.");
		}
	}

	return (
		<TableRow>
			<TableCell className="font-medium text-gray-500">New</TableCell>
			<TableCell>
				<Select value={pipelineId} onValueChange={setPipelineId}>
					<SelectTrigger className="w-[180px]">
						<SelectValue placeholder="Pipeline target" />
					</SelectTrigger>
					<SelectContent>
						{availablePipelines.map((pipeline) => (
							<SelectItem key={pipeline.id} value={pipeline.id}>
								{pipeline.name}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</TableCell>
			<TableCell>
				<Select value={method} onValueChange={setMethod}>
					<SelectTrigger>
						<SelectValue placeholder="Method" />
					</SelectTrigger>
					<SelectContent>
						<SelectGroup>
							<SelectLabel>HTTP Methods</SelectLabel>
							<SelectItem value="GET">GET</SelectItem>
							<SelectItem value="POST">POST</SelectItem>
							<SelectItem value="PUT">PUT</SelectItem>
							<SelectItem value="DELETE">DELETE</SelectItem>
							<SelectItem value="PATCH">PATCH</SelectItem>
						</SelectGroup>
					</SelectContent>
				</Select>
			</TableCell>
			<TableCell>
				<div className="flex flex-row space-x-1 items-center grow">
					<p className="font-mono">/exec/</p>
					<Input type="text" className="font-mono grow" value={path} onChange={(e) => setPath(e.target.value)} />
				</div>
			</TableCell>
			<TableCell className="text-right">
				<Button onClick={submit}>Submit</Button>
			</TableCell>
		</TableRow>
	)
}
