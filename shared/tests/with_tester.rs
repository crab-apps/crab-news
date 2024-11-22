mod tests {
    use assert_matches::assert_matches;
    use crux_core::testing::AppTester;
    use crux_http::protocol::{HttpRequest, HttpResponse, HttpResult};
    use shared::{CrabNews, Event, Model};

    #[test]
    fn get() {
        let app = AppTester::<CrabNews, _>::default();
        let mut model = Model::default();
        let gwr = "https://gentlewashrecords.com/atom.xml".to_string();

        let request = &mut app
            .update(Event::GetFeed(gwr), &mut model)
            .expect_one_effect()
            .expect_http();

        assert_eq!(request.operation, HttpRequest::get(gwr).build());

        let actual = app
            .resolve(
                request,
                HttpResult::Ok(
                    HttpResponse::ok()
                        .json("hello")
                        .header("my_header", "my_value1")
                        .header("my_header", "my_value2")
                        .build(),
                ),
            )
            .expect("Resolves successfully")
            .expect_one_event();

        assert_matches!(actual.clone(), Event::SetFeed(Ok(response)) => {
            assert_eq!(response.body().unwrap(), "\"hello\"");
            assert_eq!(response.header("my_header").unwrap().iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(), vec!["my_value1", "my_value2"]);
        });

        app.update(actual, &mut model).assert_empty();
        assert_eq!(model.body, "\"hello\"");
        assert_eq!(model.values, vec!["my_value1", "my_value2"]);
    }
}
