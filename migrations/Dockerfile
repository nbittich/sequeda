FROM node:16.19-alpine

WORKDIR /app

COPY package.json package.json
COPY package-lock.json package-lock.json 

RUN npm i

COPY . .

ENTRYPOINT [ "node", "app.js" ]