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
