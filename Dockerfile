ARG NODE_VERSION=18.15.0
FROM --platform=linux/arm64/v8 node:${NODE_VERSION}-alpine as build-stage

ENV NODE_ENV development

WORKDIR /usr/src/app

COPY src/WebUI/package.json ./WebUI/
RUN npm install --prefix ./WebUI/

COPY src/WebApi/package.json ./WebApi/
RUN npm install --prefix ./WebApi/

COPY src/WebApi ./WebApi/
COPY src/WebUI ./WebUI/

WORKDIR /usr/src/app/WebUI/
RUN npm run build 

WORKDIR /usr/src/app/WebApi/
RUN npm run build


FROM --platform=linux/arm64/v8 alpine as production-stage

RUN apk add --update nodejs

RUN addgroup -S node && adduser -S node -G node
USER node

COPY --from=build-stage /usr/src/app/WebApi/dist /app/
WORKDIR /app/

EXPOSE 8001

CMD node server.js
