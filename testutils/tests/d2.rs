use shine_testutils::init_webcontroll_test;
use shine_testutils::webserver::D2Trace;

#[test]
#[ignore]
fn test_d2() {
    let webctrl = init_webcontroll_test(module_path!());

    {
        let mut tr = D2Trace::new();

        tr.push_group_with_name("group");

        tr.push_group_with_name("points");
        tr.add_point(&(0., 0.), "black".to_string());
        tr.add_point(&(1., 0.), "red".to_string());
        tr.add_point(&(0., 1.), "green".to_string());
        tr.pop_group();

        webctrl.add_d2(tr);
    }

    {
        let mut tr = D2Trace::new();

        tr.push_group_with_name("group");

        tr.push_group_with_name("points");
        tr.add_point(&(0., 0.), "black".to_string());
        tr.add_point(&(1., 0.), "red".to_string());
        tr.add_point(&(0., 1.), "green".to_string());
        tr.pop_group();

        tr.push_group_with_name("text");
        tr.add_text(&(1., 0.), "1. red (1,0)", "red".to_string(), 1.);
        tr.add_text(&(1., 0.), "2. green (1,0)", "green".to_string(), 2.);
        tr.add_text(&(1., 0.), "3. blue (1,0)", "blue".to_string(), 0.5);
        tr.add_text(&(0., 0.), "1. red (0,0)", "red".to_string(), 1.);
        tr.add_text(&(0., 0.), "2. green (0,0)", "green".to_string(), 2.);
        tr.add_text(&(0., 0.), "3. blue (0,0)", "blue".to_string(), 0.5);
        tr.add_text(&(0., 1.), "1. red (0,1)", "red".to_string(), 0.5);
        tr.add_text(&(0., 1.), "2. green (0,1)", "green".to_string(), 1.);
        tr.add_text(&(0., 1.), "3. blue (0,1)", "blue".to_string(), 2.);
        tr.pop_group();

        tr.push_group_with_name("lines");
        tr.add_line(&(-0.2, 0.), &(-0.3, 0.), "red".to_string());
        tr.add_line(&(-0.3, 0.), &(-0.3, 0.3), "green".to_string());
        tr.add_line(&(-0.3, 0.3), &(-0.2, 0.), "yellow".to_string());
        tr.pop_group();

        tr.pop_group();

        tr.add_line(&(0.2, 0.), &(0.3, 0.), "red".to_string());
        tr.add_line(&(0.3, 0.), &(0.3, 0.3), "green".to_string());
        tr.add_line(&(0.3, 0.3), &(0.2, 0.), "yellow".to_string());

        webctrl.add_d2(tr);
    }

    webctrl.wait_user();
}
