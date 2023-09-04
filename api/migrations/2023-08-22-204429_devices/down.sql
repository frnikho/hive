-- This file should undo anything in `up.sql`

drop trigger device_before_update on devices;
drop function history_update_device;

drop table devices_history;
drop table devices_status;
drop table devices_config;
drop table devices;
drop table devices_pull_history;