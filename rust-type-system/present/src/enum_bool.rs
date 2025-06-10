mod bad {

    struct DataService {
        cache: DataCache,
        rest_api: RestService,
    }

    impl DataService {
        /// Query the data ...
        ///
        /// * `use_cache`: If enable only cache will be used, 
        ///   otherwise send REST request.
        /// * `ignore_empty`: Ignore any empty data query.
        fn query_data(&self, use_cache: bool, ignore_empty: bool) {
            let mut data = if use_cache {
                self.cache.query(...)
            }else {
                self.rest_api.query(...)
            };

            if ignore_empty {
                data.retain(...)
            }
            
            data
        }
    }

    fn example() {
        let data = service.query_data(true, false);
    }

}

mod refactoring {
    
    impl DataService {
        /// Query the data ...
        ///
        /// * `use_api`: If enable API service will be used, 
        ///   otherwise send REST request.
        /// * `ignore_empty`: Ignore any empty data query.
        fn query_data(&self, use_api: bool, ignore_empty: bool) {
            let mut data = if use_cache {
                self.cache.query(...)
            }else {
                self.rest_api.query(...)
            };

            if ignore_empty {
                data.retain(...)
            }
            
            data
        }
    }

    fn example() {
        // No Errors!
        let data = service.query_data(true, false);
    }

}


mod refactoring_2 {

    struct DataService {
        cache: DataCache,
        rest_api: RestService,
    }

    impl DataService {
        /// Query the data ...
        ///
        /// * `ignore_empty`: Ignore any empty data query.
        /// * `use_cache`: If enable only cache will be used, 
        ///   otherwise send REST request.
        fn query_data(&self, ignore_empty: bool, use_cache: bool) {
            let mut data = if use_cache {
                self.cache.query(...)
            }else {
                self.rest_api.query(...)
            };

            if ignore_empty {
                data.retain(...)
            }
            
            data
        }
    }

    fn example() {
        // No Errors!
        let data = service.query_data(true, false);
    }

}

mod good {

    enum DataSource {
        Cache,
        ApiService,
    }

    enum EmptyDataOptions {
        Ignore,
        Keep,
    }

    impl DataService {
        fn query_data(&self, source: DataSource, empty: EmptyDataOptions) {
            let mut data = match source {
                DataSource::Cache => self.cache.query(...),
                DataSource::ApiService => self.rest_api.query(...),
            };

            match empty {
                EmptyDataOptions::Ignore => data.retain(...),
                EmptyDataOptions::Keep => {},
            }
            
            data
        }
    }

    fn example() {
        let data = service.query_data(DataSource::Cache, EmptyDataOptions::Keep);
    }

}
