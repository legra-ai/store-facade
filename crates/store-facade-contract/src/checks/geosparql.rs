use store_facade_model::vocabulary::GEOSPARQL;

use crate::{
    ContractBinding,
    Probe,
};

/// OGC `GeoSPARQL` simple-features functions evaluate on WKT literals.
pub(crate) async fn geosparql<B: ContractBinding>(probe: &Probe<'_, B>) {
    let language = probe.query_language(GEOSPARQL);
    let ask = |a: &str, b: &str| {
        format!(
            "PREFIX geo: <http://www.opengis.net/ont/geosparql#> \
             PREFIX geof: <http://www.opengis.net/def/function/geosparql/> \
             ASK {{ FILTER(geof:sfIntersects(\"{a}\"^^geo:wktLiteral, \"{b}\"^^geo:wktLiteral)) }}"
        )
    };
    assert!(
        probe
            .ask(
                language,
                &ask("POINT(1 1)", "POLYGON((0 0, 2 0, 2 2, 0 2, 0 0))")
            )
            .await
    );
    assert!(
        !probe
            .ask(
                language,
                &ask("POINT(5 5)", "POLYGON((0 0, 2 0, 2 2, 0 2, 0 0))")
            )
            .await
    );
}
