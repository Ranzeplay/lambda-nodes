# Lambda Nodes

Lambda Nodes is a modern, full-stack application for managing pipelines, nodes, and routes. It provides a user-friendly interface for creating, editing, and monitoring workflows.

## Subprojects

- [Studio](./studio/README.md): The frontend application built with React.
- [Agent](./agent/README.md): The backend application built with Rust.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/lambda-nodes.git
   cd lambda-nodes
   ```

2. Install dependencies for the Studio:
   ```bash
   cd studio
   pnpm install
   ```

3. Install dependencies for the Agent:
   ```bash
   cd ../agent
   cargo build
   ```

## Running the Project

### Studio (Frontend)

1. Navigate to the `studio` directory:
   ```bash
   cd studio
   ```

2. Start the development server:
   ```bash
   pnpm run dev
   ```

3. Open your browser and navigate to `http://localhost:5173`.

### Agent (Backend)

1. Navigate to the `agent` directory:
   ```bash
   cd agent
   ```

2. Start the backend server:
   ```bash
   cargo run
   ```

3. The backend will be available at `http://localhost:3000`.

## Building for Production

### Studio

1. Navigate to the `studio` directory:
   ```bash
   cd studio
   ```

2. Create a production build:
   ```bash
   pnpm run build
   ```

3. Serve the build using a static file server or deploy it to a hosting platform.

### Agent

1. Navigate to the `agent` directory:
   ```bash
   cd agent
   ```

2. Build the release version:
   ```bash
   cargo build --release
   ```

3. Deploy the binary to your server.

## Deployment

### Docker Deployment

1. Build the Docker image:
   ```bash
   docker build -t lambda-nodes .
   ```

2. Run the container:
   ```bash
   docker run -p 3000:3000 lambda-nodes
   ```

3. Access the application at `http://localhost:3000`.


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
