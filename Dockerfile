ARG NODE_VERSION=22
FROM node:${NODE_VERSION}-alpine AS build-stage

ENV NODE_ENV development

WORKDIR /usr/src/app

RUN npm install -g pnpm

COPY src/WebUI/package.json ./WebUI/
RUN pnpm install --prefix ./WebUI/

COPY src/WebApi/package.json ./WebApi/
RUN pnpm install --prefix ./WebApi/

COPY src/WebApi ./WebApi/
COPY src/WebUI ./WebUI/

WORKDIR /usr/src/app/WebUI/
RUN pnpm run build 

WORKDIR /usr/src/app/WebApi/
RUN pnpm run build


FROM alpine AS production-stage

RUN apk add --update nodejs

RUN addgroup -S node && adduser -S node -G node
USER node

COPY --from=build-stage /usr/src/app/WebApi/dist /app/
WORKDIR /app/

EXPOSE 8101 8102

CMD node server.js
