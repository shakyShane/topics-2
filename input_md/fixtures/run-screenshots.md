# Action: Run screenshot tests

## Dependencies

Please don't skip the dependencies below - open each link and follow through.

1. Set up machine for Kubernetes 
2. NPM rc configuration

## Step 1: Command: Build the images

The screenshot testing process requires at least the following 2 images to be built before they are run.

```
client
disconnectedlayoutservice
```

You can build these with the following command

```shell
skaffold build -b client -b screenshots 
```

## Step 2: Command: Run the tests

```shell
skaffold build -b "client" -b "screenshots" 
```

```toml
cwd = { valueFrom = "config", name = "Global", path = "paths.client" }
```