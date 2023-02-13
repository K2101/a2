use rdkafka::consumer::{ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::producer::FutureProducer;
use rdkafka::{ClientConfig, ClientContext, TopicPartitionList};

pub async fn producer_client(broker: &str) -> FutureProducer {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", broker)
        // .set("message.timeout.ms", "5000")
        // production setting
        // .set("min.insync.replicas", "2")
        // max.in.flight.requests.per.connection = 1 ?? for ordering in the producer
        .set("acks", "all") // this is default for kafka but for also for redpanda ??
        .create()
        .expect("Cannot create producer client");

    println!("Created producer");
    producer
}

pub struct CustomConsumerContext;
impl ClientContext for CustomConsumerContext {}

impl ConsumerContext for CustomConsumerContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, offsets: &TopicPartitionList) {
        println!("Committing offsets: {:?}, offset: {:?}", result, offsets);
    }
}

pub type LoggingConsumer = StreamConsumer<CustomConsumerContext>;

pub async fn consumer_client(broker: &str, grop_id: &str) -> LoggingConsumer {
    let context = CustomConsumerContext;
    let consumer: LoggingConsumer = ClientConfig::new()
        .set("bootstrap.servers", broker)
        .set("group.id", grop_id)
        // or false   .set("enable.auto.commit", "false")
        .set("enable.auto.commit", "true")
        // default for kafka
        // Commit automatically every 5 seconds.
        .set("auto.commit.interval.ms", "5000")
        // https://docs.confluent.io/3.2.1/clients/librdkafka/CONFIGURATION_8md.html
        .set("enable.auto.offset.store", "false")
        // https://stackoverflow.com/questions/32390265/what-determines-kafka-consumer-offset
        .set("auto.offset.reset", "earliest")
        // production setting
        .create_with_context(context)
        .expect("Cannot create consumer client");

    println!("Created consumer");
    consumer
}
