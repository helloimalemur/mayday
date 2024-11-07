FROM node:22.4.0-bullseye
WORKDIR /mayday-fe
COPY frontend/ .
RUN ls
ENV FRONTEND_PORT=${FRONTEND_PORT}
ENV NODE_VERSION=22.4.0
RUN apt install -y curl
RUN node --version
RUN npm --version
RUN npm install --verbose
RUN ls -lah
RUN ./node_modules/next/dist/bin/next build
CMD ["./start-container.sh"]
EXPOSE $FRONTEND_PORT
