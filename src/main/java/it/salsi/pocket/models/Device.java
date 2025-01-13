/*
The MIT License (MIT)

Original Work Copyright (c) 2018-2025 Antonio Salsi

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

package it.salsi.pocket.models;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.fasterxml.jackson.annotation.JsonIgnore;
import jakarta.persistence.*;
import jakarta.validation.constraints.NotEmpty;
import jakarta.validation.constraints.NotNull;
import jakarta.validation.constraints.Size;
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;

import java.time.ZoneOffset;
import java.time.ZonedDateTime;
import java.util.Date;

import static it.salsi.pocket.Constant.DATE_TIME_FORMAT;
import static it.salsi.pocket.models.Device.Status.ACTIVE;

@ToString
//@Data no perchè crash OneToMany
@Entity(name = "devices")
@Table(
        indexes = {@Index(name = "idx_devices_device_serial", columnList = "deviceSerial"), @Index(name = "idx_devices_token", columnList = "token")},
        uniqueConstraints = {@UniqueConstraint(columnNames = {"token"})}
)
@Getter
@Setter
@SuppressWarnings("JpaDataSourceORMInspection")
public final class Device {


    @SuppressWarnings("unused")
    public enum Status {
        UNACTIVE, ACTIVE, DELETED, INVALIDATED
    }

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id = 0L;

    @Size(max = 256, message = "max size exceeded; maximum 256 char")
    @NotEmpty(message = "field empty")
    @NotNull(message = "field null")
    @Column(nullable = false)
    private String deviceSerial = "";

    @JsonIgnore
    private String version;

    @Column(nullable = false)
    private String token = "";

    @JsonIgnore
    @Temporal(TemporalType.TIMESTAMP)
    private Date dateTimeLastUpdate = null;

    @JsonFormat(pattern = DATE_TIME_FORMAT, timezone = "UTC")
    @Temporal(TemporalType.TIMESTAMP)
    @Column(nullable = false)
    private Date dateTimeLastLogin = Date.from(ZonedDateTime.now(ZoneOffset.UTC).toInstant());

    @JsonIgnore
    private String address;

    @Column(nullable = false)
    private Status status = ACTIVE;

    @ToString.Exclude
    @JoinColumn(name = "user_id", referencedColumnName = "id")
    @ManyToOne(fetch = FetchType.EAGER)
    @JsonIgnore
    private User user;

    public void updateDateTimeLastLogin() {
        dateTimeLastLogin = Date.from(ZonedDateTime.now(ZoneOffset.UTC).toInstant());
    }

    public void updateDateTimeLastUpdate() {
        if (user.getDateTimeLastUpdate() != null) {
            dateTimeLastUpdate = user.getDateTimeLastUpdate();
        }
    }
}