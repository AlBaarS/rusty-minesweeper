# Use an official Node.js image
FROM node:23-alpine AS builder

WORKDIR /app

# Install dependencies separately to leverage Docker caching
COPY package.json package-lock.json ./
RUN npm install

# Copy the rest of the app and build
COPY . .
RUN npm run build

# Use a minimal runtime image
FROM node:23-alpine

WORKDIR /app

# Copy the built app
COPY --from=builder /app ./

# Expose the required port
EXPOSE 3000

# Start the Next.js server
CMD ["npm", "run", "start"]