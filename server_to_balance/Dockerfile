FROM node:18-alpine

WORKDIR /usr/src/app

RUN npm install -g pnpm
RUN npm install -g typescript

COPY package*.json ./
RUN pnpm install

COPY . .

EXPOSE 8000
RUN tsc -b
CMD [ "node", "dist/index.js" ]
