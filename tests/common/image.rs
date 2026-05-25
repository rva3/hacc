use hacc::*;

const LK_IMAGE: &[u8] = include_bytes!("../../tests/files/lk.img");

#[test]
fn image_header_parse() {
    let image = Image::new(LK_IMAGE);

    let mock1 = image.get_partition("mock").expect("Failed to parse first partition");

    assert!(mock1.header.is_valid(), "Partition header magic or header size is invalid");
    assert!(mock1.header.is_extended());
    assert!(mock1.header.size() > 0, "Partition size should be greater than 0");
    assert_eq!(mock1.header.data_size(), mock1.content.len() as u64);
    assert_eq!(mock1.header.name(), "mock");
    assert_eq!(mock1.header.image_id().unwrap(), ImageKind::Ap(ImageAPKind::APBin));
    assert_eq!(image.get_part_certs("mock").count(), 0);

    let mock2 = image.get_partition("mock2").expect("Failed to parse second partition");
    assert!(mock2.header.is_valid(), "Partition header magic or header size is invalid");
    assert!(mock2.header.is_extended());
    assert!(mock2.header.size() > 0, "Partition size should be greater than 0");
    assert_eq!(mock2.header.data_size(), mock2.content.len() as u64);
    assert_eq!(mock2.header.name(), "mock2");
    assert_eq!(mock2.header.image_id().unwrap(), ImageKind::Ap(ImageAPKind::APBin));
    assert_eq!(image.get_part_certs("mock2").count(), 2);

    assert!(image.get_partition("non_existent").is_none());
}

#[test]
fn image_from_invalid_bytes() {
    let image = Image::new(&[]);
    assert!(image.get_partition("mock").is_none());

    let data = [0xFF; 4096];
    let image = Image::new(&data);
    assert!(image.get_partition("mock").is_none());

    let truncated = &LK_IMAGE[..LK_IMAGE.len() / 2];
    let image = Image::new(truncated);

    assert!(image.get_partition("mock2").is_none());
}

#[cfg(feature = "alloc")]
#[test]
fn image_add_partition() {
    let mut image = Image::new(LK_IMAGE);
    let kind = ImageKind::Md(ImageMDKind::MdLte);
    image.add_partition("test_part", b"test_content", kind).unwrap();

    assert!(image.has_partition("test_part"));

    let partition = image.get_partition("test_part").expect("Failed to get added partition");

    assert!(partition.header.is_valid(), "Partition header magic or header size is invalid");
    assert!(partition.header.is_extended());
    assert_eq!(partition.header.data_size(), "test_content".len() as u64);
    assert_eq!(partition.header.name(), "test_part");
    assert_eq!(partition.header.image_id().unwrap(), ImageKind::Md(ImageMDKind::MdLte));

    let mut image = Image::new(LK_IMAGE);

    let result = image.add_partition("", b"", ImageKind::Md(ImageMDKind::MdLte));

    assert!(result.is_err(), "Adding partition with empty name should fail");
    assert!(matches!(result, Err(Error::Image(ImageError::PartitionNameEmpty))));

    let result = image.add_partition("test_part", b"", ImageKind::Md(ImageMDKind::MdLte));

    assert!(result.is_err(), "Adding partition with empty content should fail");
    assert!(matches!(result, Err(Error::Image(ImageError::PartitionContentEmpty))));

    let name = "A".repeat(33);

    let result = image.add_partition(&name, b"test_content", ImageKind::Md(ImageMDKind::MdLte));

    assert!(result.is_err(), "Adding partition with name longer than 32 characters should fail");
    assert!(matches!(result, Err(Error::Image(ImageError::PartitionNameTooLong))));
}

#[cfg(feature = "alloc")]
#[test]
fn image_remove_partition() {
    let mut image = Image::new(LK_IMAGE);

    assert!(image.has_partition("mock"));

    image.remove_partition("mock").unwrap();

    assert!(!image.has_partition("mock"));
}
