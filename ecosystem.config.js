module.exports = {
  apps: [
    {
      name: "server",
      script: "./target/release/server"
    },
    {
      name: "oracle",
      script: "./target/release/oracle"
    },
    {
      name: "scanner",
      script: "./target/release/scanner"
    }
  ]
};
