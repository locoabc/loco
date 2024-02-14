+++
title = "Workers"
description = ""
date = 2021-05-01T18:10:00+00:00
updated = 2021-05-01T18:10:00+00:00
draft = false
weight = 15
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ""
toc = true
top = false
flair =[]
+++

`loco` integrates with a full blown background job processing framework: `sidekiq-rs`. You can enqueue jobs in a similar ergonomics as Rails' _ActiveJob_, and have a similar scalable processing model to perform these background jobs.

## Using workers

To use a worker, we mainly think about adding a job to the queue, so you `use` the worker and perform later:

```rust
    // .. in your controller ..
    DownloadWorker::perform_later(
        &ctx,
        DownloadWorkerArgs {
            user_guid: "foo".to_string(),
        },
    )
    .await
```

Unlike Rails and Ruby, with Rust you can enjoy _strongly typed_ job arguments which gets serialized and pushed into the queue.

## Adding a worker

Adding a worker meaning coding the background job logic to take the _arguments_ and perform a job. We also need to let `loco` know about it and register it into the global job processor.

Add a worker to `workers/`:

```rust
#[async_trait]
impl Worker<DownloadWorkerArgs> for DownloadWorker {
    async fn perform(&self, args: DownloadWorkerArgs) -> Result<()> {
        println!("================================================");
        println!("Sending payment report to user {}", args.user_guid);

        // TODO: Some actual work goes here...

        println!("================================================");
        Ok(())
    }
}
```

And register it in `app.rs`:

```rust
#[async_trait]
impl Hooks for App {
//..
    fn connect_workers<'a>(p: &'a mut Processor, ctx: &'a AppContext) {
        p.register(DownloadWorker::build(ctx));
    }
// ..
}
```

## Generate a Worker

To automatically add a worker using `loco generate`, execute the following command:

```sh
cargo loco generate worker report_worker
```

The worker generator creates a worker file associated with your app and generates a test template file, enabling you to verify your worker.

## Configuring Workers

In your `config/<environment>.yaml` you can specify the worker mode. BackgroundAsync and BackgroundQueue will process jobs in a non-blocking manner, while ForegroundBlocking will process jobs in a blocking manner.

The main difference between BackgroundAsync and BackgroundQueue is that the latter will use Redis to store the jobs, while the former does not require Redis and will use async within the same process.

```yaml
# Worker Configuration
workers:
  # specifies the worker mode. Options:
  #   - BackgroundQueue - Workers operate asynchronously in the background, processing queued.
  #   - ForegroundBlocking - Workers operate in the foreground and block until tasks are completed.
  #   - BackgroundAsync - Workers operate asynchronously in the background, processing tasks with async capabilities.
  mode: BackgroundQueue
```

### Testing a Worker

For comprehensive testing of integrated background jobs, please refer to the detailed documentation [here](@/docs/testing/worker.md)
