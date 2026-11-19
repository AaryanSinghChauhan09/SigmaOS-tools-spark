#![forbid(unsafe_code)]
#![allow(non_snake_case)]
//! Apache Spark RDD Engine for SigmaOS

pub struct SparkRdd<T> {
    data: Vec<T>,
}

impl<T: Clone + 'static> SparkRdd<T> {
    pub fn parallelize(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn map<U: Clone + 'static, F: Fn(T) -> U>(self, f: F) -> SparkRdd<U> {
        let mapped = self.data.into_iter().map(f).collect();
        SparkRdd { data: mapped }
    }

    pub fn filter<F: Fn(&T) -> bool>(self, predicate: F) -> SparkRdd<T> {
        let filtered = self.data.into_iter().filter(predicate).collect();
        SparkRdd { data: filtered }
    }

    pub fn reduce<F: Fn(T, T) -> T>(self, f: F) -> Option<T> {
        self.data.into_iter().reduce(f)
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn collect(self) -> Vec<T> {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spark_rdd_transformations_and_actions_aaa() {
        // Arrange
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let rdd = SparkRdd::parallelize(numbers);

        // Act
        let result = rdd
            .filter(|&n| n % 2 == 0) // Keep evens: [2, 4, 6, 8, 10]
            .map(|n| n * 10) // Multiply by 10: [20, 40, 60, 80, 100]
            .reduce(|a, b| a + b); // Sum: 300

        // Assert
        assert_eq!(result, Some(300));
    }
}


#[cfg(test)]
mod spark_extra_tests {
    use super::*;

    #[test]
    fn test_spark_collect_preserves_data_aaa() {
        // Arrange
        let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let rdd = SparkRdd::parallelize(items.clone());
        // Act
        let collected = rdd.collect();
        // Assert
        assert_eq!(collected, items);
    }

    #[test]
    fn test_spark_empty_rdd_count_and_reduce_aaa() {
        // Arrange
        let rdd: SparkRdd<i32> = SparkRdd::parallelize(Vec::new());
        // Act
        let cnt = rdd.count();
        let red = rdd.reduce(|a, b| a + b);
        // Assert
        assert_eq!(cnt, 0);
        assert_eq!(red, None);
    }

    #[test]
    fn test_spark_filter_all_out_aaa() {
        // Arrange
        let rdd = SparkRdd::parallelize(vec![1, 3, 5]);
        // Act
        let evens = rdd.filter(|n| n % 2 == 0);
        // Assert
        assert_eq!(evens.count(), 0);
    }
}
