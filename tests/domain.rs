/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <http://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

extern crate virt;

mod common;

use virt::domain::Domain;


fn tdom(exec_test: fn(dom: Domain)) {
    let c = common::conn();
    match Domain::lookup_by_name(&c, "test") {
        Ok(dom) => {
            exec_test(dom);
            common::close(c);
        }
        Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
    }
}

#[test]
fn test_name() {
    fn t(dom: Domain) {
        assert_eq!("test", dom.get_name().unwrap_or(String::new()));
    }
    tdom(t);
}

#[test]
fn test_uuid_string() {
    fn t(dom: Domain) {
        assert_eq!("6695eb01-f6a4-8304-79aa-97f2502e193f",
                   dom.get_uuid_string().unwrap_or(String::new()));
    }
    tdom(t);
}

#[test]
fn test_id() {
    fn t(dom: Domain) {
        assert_eq!(1, dom.get_id().unwrap_or(0));
    }
    tdom(t);
}

#[test]
fn test_get_xml_desc() {
    fn t(dom: Domain) {
        assert!("" != dom.get_xml_desc(0).unwrap_or(String::new()),
                "Should not be empty");
    }
    tdom(t);
}

#[test]
fn test_get_info() {
    fn t(dom: Domain) {
        match dom.get_info() {
            Ok(info) => assert_eq!(1, info.state),
            Err(_) => panic!("should have a node info"),
        }
    }
    tdom(t);
}

#[test]
fn test_get_vcpus_flags() {
    fn t(dom: Domain) {
        assert_eq!(2, dom.get_vcpus_flags(0).unwrap_or(0));
    }
    tdom(t);
}

#[test]
fn test_lookup_domain_by_id() {
    let c = common::conn();
    let v = c.list_domains().unwrap_or(vec![]);
    assert!(0 < v.len(), "At least one domain should exist");
    for domid in v {
        match Domain::lookup_by_id(&c, domid) {
            Ok(mut dom) => dom.free().unwrap_or(()),
            Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
        }
    }
    common::close(c);
}

#[test]
fn test_lookup_domain_by_name() {
    let c = common::conn();
    match Domain::lookup_by_name(&c, "test") {
        Ok(mut r) => r.free().unwrap_or(()),
        Err(e) => panic!("failed with code {}, message: {}", e.code, e.message),
    }
    common::close(c);
}
