use oxidrive_paginate::{Paginate, Slice};

pub fn to_slice<T, F>(items: Vec<T>, cursor: F, paginate: &Paginate) -> Slice<T>
where
    F: FnOnce(&T) -> String,
{
    let cursor = items.last().map(cursor);

    if paginate.is_forward() {
        Slice::new(items, cursor, None)
    } else {
        Slice::new(items, None, cursor)
    }
}

pub mod postgres {
    use oxidrive_paginate::Paginate;
    use sqlx::{types::Uuid, Postgres, QueryBuilder};

    pub fn push_query<'a>(
        qb: &mut QueryBuilder<'a, Postgres>,
        paginate: &'a Paginate,
        order_by: &'static str,
    ) {
        let order_and_limit = format!(" order by {order_by} limit ");

        match paginate {
            Paginate::Forward { after, first } => {
                let limit = *first as i64;

                qb.push(" and id::text >");

                if after.is_empty() {
                    qb.push_bind(Uuid::nil().to_string());
                } else {
                    qb.push_bind(after);
                };

                qb.push(order_and_limit).push_bind(limit);
            }

            Paginate::Backward { before, last } => {
                let limit = *last as i64;

                qb.push(" and id::text < ");

                if before.is_empty() {
                    qb.push_bind(Uuid::max().to_string());
                } else {
                    qb.push_bind(before);
                };

                qb.push(order_and_limit).push_bind(limit);
            }
        }
    }
}

pub mod sqlite {
    use oxidrive_paginate::Paginate;
    use sqlx::{types::Uuid, QueryBuilder, Sqlite};

    pub fn push_query<'a>(
        qb: &mut QueryBuilder<'a, Sqlite>,
        paginate: &'a Paginate,
        order_by: &'static str,
    ) {
        let order_and_limit = format!(" order by {order_by} limit ");

        match paginate {
            Paginate::Forward { after, first } => {
                let limit = *first as i64;

                qb.push(" and id >");

                if after.is_empty() {
                    qb.push_bind(Uuid::nil().to_string());
                } else {
                    qb.push_bind(after);
                };

                qb.push(order_and_limit).push_bind(limit);
            }

            Paginate::Backward { before, last } => {
                let limit = *last as i64;

                qb.push(" and id < ");

                if before.is_empty() {
                    qb.push_bind(Uuid::max().to_string());
                } else {
                    qb.push_bind(before);
                };

                qb.push(order_and_limit).push_bind(limit);
            }
        }
    }
}
