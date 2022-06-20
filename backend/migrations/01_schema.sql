create table manufacturer (
    manufacturer_id integer primary key autoincrement not null,
    manufacturer_name text not null unique
);

create table sauce (
    sauce_id integer primary key autoincrement not null,
    sauce_name text not null,
    manufacturer_id integer not null,
    foreign key (manufacturer_id) references manufacturer (manufacturer_id)
);

create table user (
    user_id integer primary key autoincrement not null,

    -- Eg "Joe Roberts"
    full_name text not null,

    -- Eg "Joe"
    short_name text not null,
    
    signup_timestamp timestamp not null
);

-- A review is a bundle of ratings/comments/etc.. made by a single user at a
-- single time. A user may submit multiple reviews of the same sauce at
-- different times.
create table review (
    review_id integer primary key autoincrement not null,
    user_id integer not null,
    sauce_id integer not null,
    review_timestamp timestamp default current_timestamp not null
);

create table rating_axis (
    rating_axis_id integer primary key autoincrement not null,
    rating_axis_name text not null,

    -- A human readable text description of what a low value of this axis represents
    -- Eg for "heat", this might be "Not at all hot"
    min_value_desc text not null,

    -- A human readable text description of what a low value of this axis represents
    -- Eg for "heat", this might be "The hottest thing you've ever experienced"
    max_value_desc text not null
);

create table review_rating (
    rating_id integer primary key autoincrement not null,
    review_id integer not null,
    rating_axis_id integer not null,
    rating real not null,

    foreign key (review_id) references review (review_id),
    foreign key (rating_axis_id) references rating_axis (rating_axis_id)
);

insert into rating_axis (rating_axis_name, min_value_desc, max_value_desc)
values
    ("Overall", "Strong dislike for this sauce", "The best sauce ever"),
    ("Heat", "Not at all hot", "The hottest thing you've ever experienced"),
    ("Flavour", "Actively unpleasant", "Truly delicious");

insert into manufacturer (manufacturer_name) values
    ("Mahi"),
    ("Bravado Spice Company"),
    ("Torchbearer"),
    ("Wiltshire Chilli Farm");

insert into sauce (sauce_name, manufacturer_id) values
    ("Scorpion Pepper and Passion Fruit", (select manufacturer_id from manufacturer where manufacturer_name = "Mahi")),
    ("Aka Miso Ghost-Reaper", (select manufacturer_id from manufacturer where manufacturer_name = "Bravado Spice Company"));