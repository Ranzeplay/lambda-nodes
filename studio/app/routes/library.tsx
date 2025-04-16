import { Link } from "react-router";
import FrameView from "~/components/frame";
import { Button } from "~/components/ui/button";
import { TableHeader, TableRow, TableHead, TableBody, TableCell, Table } from "~/components/ui/table";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "~/components/ui/tooltip";
import { Binoculars, Pencil, Plus } from "lucide-react";
import { Input } from "~/components/ui/input";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { fetchAllNodes, type NodeIndexData } from "~/lib/nodeModel";
import { Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle, DrawerTrigger } from "~/components/ui/drawer";
import MonacoEditorWrapper from "~/components/monacoEditorWrapper";

export default function LibraryPage() {
	const [nodes, setNodes] = useState<NodeIndexData[]>([]);

	useEffect(() => {
		fetchAllNodes()
			.then((data) => {
				setNodes(data!);
			})
			.catch((error) => {
				toast.error("Failed to fetch nodes. Please try again.");
				console.error("Error fetching nodes:", error);
			});
	}, []);

	return (
		<FrameView title="Library" subtitle="All scripts">
			<div className="flex flex-col space-y-4">
				<div className="flex flex-row space-x-4">
					<Input type="text" placeholder="Search..." className="grow" />
					<Button>
						<Link to="/node/new" className="flex items-center gap-x-2 m-0 p-0">
							<Plus /> New
						</Link>
					</Button>
				</div>

				<Table>
					<TableHeader>
						<TableRow>
							<TableHead className="font-bold w-[120px]">Id</TableHead>
							<TableHead className="font-bold">Name</TableHead>
							<TableHead className="font-bold">In data count</TableHead>
							<TableHead className="font-bold">Out data count</TableHead>
							<TableHead className="font-bold text-right pr-6">Operations</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{nodes.map((node) => (
							<TableRow>
								<TableCell className="font-medium">{node.id}</TableCell>
								<TableCell className={`${node.isInternal && 'italic'}`}>{node.name}</TableCell>
								<TableCell>{node.inputs.length}</TableCell>
								<TableCell>{node.outputs.length}</TableCell>
								<TableCell className="text-right">
									{!node.isInternal && (
										<div className="flex flex-row justify-end">
											<Button variant="link" className="text-blue-500 hover:text-blue-700 !m-0 hover:border *:cursor-pointer" asChild>
												<Link to={`/node/edit/${node.id}`}>
													<TooltipProvider>
														<Tooltip>
															<TooltipTrigger><Pencil /></TooltipTrigger>
															<TooltipContent>
																<p>Edit</p>
															</TooltipContent>
														</Tooltip>
													</TooltipProvider>
												</Link>
											</Button>
											<Drawer direction="right">
												<DrawerTrigger asChild>
													<Button variant="link" className="text-blue-500 hover:text-blue-700 !m-0 hover:border *:cursor-pointer">
														<TooltipProvider>
															<Tooltip>
																<TooltipTrigger><Binoculars /></TooltipTrigger>
																<TooltipContent>
																	<p>Inspect</p>
																</TooltipContent>
															</Tooltip>
														</TooltipProvider>
													</Button>
												</DrawerTrigger>
												<DrawerContent>
													<DrawerHeader>
														<DrawerTitle>Inspect node</DrawerTitle>
														<DrawerDescription>{node.id}</DrawerDescription>
													</DrawerHeader>

													<div className="flex flex-col space-y-2 mx-4">
														<hr className="border border-gray-300" />
														<div className="flex flex-col space-y-1">
															<h3 className="font-semibold">Name</h3>
															<p className="font-mono ml-2 text-gray-600">{node.name}</p>
														</div>
														<div className="flex flex-col space-y-1">
															<h3 className="font-semibold">Input data</h3>
															{node.inputs.length === 0 ?
																<p className="font-mono ml-2 text-gray-600 italic">No input data</p> :
																(
																	<ul className="list-disc ml-6">
																		{node.inputs.map((input, index) => (
																			<li key={index} className="font-mono text-gray-600">{input}</li>
																		))}
																	</ul>
																)
															}
														</div>
														<div className="flex flex-col space-y-1">
															<h3 className="font-semibold">Output data</h3>
															{node.outputs.length === 0 ?
																<p className="font-mono ml-2 text-gray-600 italic">No output data</p> :
																(
																	<ul className="list-disc ml-6">
																		{node.outputs.map((output, index) => (
																			<li key={index} className="font-mono text-gray-600">{output}</li>
																		))}
																	</ul>
																)
															}
														</div>
														<div className="flex flex-col space-y-1">
															<h3 className="font-semibold">Script</h3>
															<MonacoEditorWrapper
																value={node.script ?? "<unknown>"}
																language="javascript"
																options={{
																	readOnly: true,
																	minimap: { enabled: false },
																	scrollBeyondLastLine: false,
																	automaticLayout: true,
																}}
																height={"18vh"}
																className="border border-gray-300"
															/>
														</div>
													</div>
													<DrawerFooter className="flex flex-row-reverse">
														<DrawerClose className="flex">
															<Button variant="outline">Close</Button>
														</DrawerClose>
													</DrawerFooter>
												</DrawerContent>
											</Drawer>
										</div>
									)}
								</TableCell>
							</TableRow>
						))}
					</TableBody>
				</Table>
			</div>
		</FrameView>
	);
}
