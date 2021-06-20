# もろもろ




## Hello Tokio

- 127.0.0.1:6379 はmini-redis-serverのアドレス
- "".into()　&strをStringにコンバート

- async fn はFuture traitを実装した無名の型を返す
- async fn は .awaitによって初めて実行される．

## Spawining

- 並行(Concurrency)と並列(Parallelism)の違い
    - Concurrency は交互に作業を行うこと　実際の例では，CPUが行う作業がCocurrencyに相当する（実際はParallelismも行うが）．作業者が多くいるわけではない(GPUと比べて)が，1人が行う作業の速度が早いため（CPUの周波数），交互に作業を行ってもそれが成り立つ．
    - Parallelism は２つ以上の作業者が同時に作業を行うこと　GPUではPrallelismであり，それは一つ一つのコアの処理性能が圧倒的に優れているわけではないが，コア(作業者)が多く(CPUと比べて)，同時に作業が行える．



## Shared state


## Channels


## I/O

## Framing


## Async in depth

## Select


## Streams
