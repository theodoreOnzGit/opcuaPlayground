/* ========================================================================
 * Copyright (c) 2005-2021 The OPC Foundation, Inc. All rights reserved.
 *
 * OPC Foundation MIT License 1.00
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation
 * files (the "Software"), to deal in the Software without
 * restriction, including without limitation the rights to use,
 * copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following
 * conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * The complete license agreement can be found here:
 * http://opcfoundation.org/License/MIT/1.00/
 * ======================================================================*/

using System;
using System.Collections.Generic;
using System.Text;
using System.Xml;
using System.Runtime.Serialization;
using Opc.Ua;

namespace MyModel
{
    #region temperature_reading Class
    #if (!OPCUA_EXCLUDE_temperature_reading)
    /// <summary>
    /// 
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    [DataContract(Namespace = MyModel.Namespaces.MyModel)]
    public partial class temperature_reading : IEncodeable
    {
        #region Constructors
        /// <summary>
        /// The default constructor.
        /// </summary>
        public temperature_reading()
        {
            Initialize();
        }

        /// <summary>
        /// Called by the .NET framework during deserialization.
        /// </summary>
        [OnDeserializing]
        private void Initialize(StreamingContext context)
        {
            Initialize();
        }

        /// <summary>
        /// Sets private members to default values.
        /// </summary>
        private void Initialize()
        {
            m_temperatureValueC = (double)0;
            m_unit = null;
        }
        #endregion

        #region Public Properties
        /// <remarks />
        [DataMember(Name = "TemperatureValueC", IsRequired = false, Order = 1)]
        public double TemperatureValueC
        {
            get { return m_temperatureValueC;  }
            set { m_temperatureValueC = value; }
        }

        /// <remarks />
        [DataMember(Name = "Unit", IsRequired = false, Order = 2)]
        public string Unit
        {
            get { return m_unit;  }
            set { m_unit = value; }
        }
        #endregion

        #region IEncodeable Members
        /// <summary cref="IEncodeable.TypeId" />
        public virtual ExpandedNodeId TypeId
        {
            get { return DataTypeIds.temperature_reading; }
        }

        /// <summary cref="IEncodeable.BinaryEncodingId" />
        public virtual ExpandedNodeId BinaryEncodingId
        {
            get { return ObjectIds.temperature_reading_Encoding_DefaultBinary; }
        }

        /// <summary cref="IEncodeable.XmlEncodingId" />
        public virtual ExpandedNodeId XmlEncodingId
        {
            get { return ObjectIds.temperature_reading_Encoding_DefaultXml; }
        }

        /// <summary cref="IEncodeable.Encode(IEncoder)" />
        public virtual void Encode(IEncoder encoder)
        {
            encoder.PushNamespace(MyModel.Namespaces.MyModel);

            encoder.WriteDouble("TemperatureValueC", TemperatureValueC);
            encoder.WriteString("Unit", Unit);

            encoder.PopNamespace();
        }

        /// <summary cref="IEncodeable.Decode(IDecoder)" />
        public virtual void Decode(IDecoder decoder)
        {
            decoder.PushNamespace(MyModel.Namespaces.MyModel);

            TemperatureValueC = decoder.ReadDouble("TemperatureValueC");
            Unit = decoder.ReadString("Unit");

            decoder.PopNamespace();
        }

        /// <summary cref="IEncodeable.IsEqual(IEncodeable)" />
        public virtual bool IsEqual(IEncodeable encodeable)
        {
            if (Object.ReferenceEquals(this, encodeable))
            {
                return true;
            }

            temperature_reading value = encodeable as temperature_reading;

            if (value == null)
            {
                return false;
            }

            if (!Utils.IsEqual(m_temperatureValueC, value.m_temperatureValueC)) return false;
            if (!Utils.IsEqual(m_unit, value.m_unit)) return false;

            return true;
        }

        #if !NET_STANDARD
        /// <summary cref="ICloneable.Clone" />
        public virtual object Clone()
        {
            return (temperature_reading)this.MemberwiseClone();
        }
        #endif

        /// <summary cref="Object.MemberwiseClone" />
        public new object MemberwiseClone()
        {
            temperature_reading clone = (temperature_reading)base.MemberwiseClone();

            clone.m_temperatureValueC = (double)Utils.Clone(this.m_temperatureValueC);
            clone.m_unit = (string)Utils.Clone(this.m_unit);

            return clone;
        }
        #endregion

        #region Private Fields
        private double m_temperatureValueC;
        private string m_unit;
        #endregion
    }

    #region temperature_readingCollection Class
    /// <summary>
    /// A collection of temperature_reading objects.
    /// </summary>
    /// <exclude />
    [System.CodeDom.Compiler.GeneratedCodeAttribute("Opc.Ua.ModelCompiler", "1.0.0.0")]
    [CollectionDataContract(Name = "ListOftemperature_reading", Namespace = MyModel.Namespaces.MyModel, ItemName = "temperature_reading")]
    #if !NET_STANDARD
    public partial class temperature_readingCollection : List<temperature_reading>, ICloneable
    #else
    public partial class temperature_readingCollection : List<temperature_reading>
    #endif
    {
        #region Constructors
        /// <summary>
        /// Initializes the collection with default values.
        /// </summary>
        public temperature_readingCollection() {}

        /// <summary>
        /// Initializes the collection with an initial capacity.
        /// </summary>
        public temperature_readingCollection(int capacity) : base(capacity) {}

        /// <summary>
        /// Initializes the collection with another collection.
        /// </summary>
        public temperature_readingCollection(IEnumerable<temperature_reading> collection) : base(collection) {}
        #endregion

        #region Static Operators
        /// <summary>
        /// Converts an array to a collection.
        /// </summary>
        public static implicit operator temperature_readingCollection(temperature_reading[] values)
        {
            if (values != null)
            {
                return new temperature_readingCollection(values);
            }

            return new temperature_readingCollection();
        }

        /// <summary>
        /// Converts a collection to an array.
        /// </summary>
        public static explicit operator temperature_reading[](temperature_readingCollection values)
        {
            if (values != null)
            {
                return values.ToArray();
            }

            return null;
        }
        #endregion

        #if !NET_STANDARD
        #region ICloneable Methods
        /// <summary>
        /// Creates a deep copy of the collection.
        /// </summary>
        public object Clone()
        {
            return (temperature_readingCollection)this.MemberwiseClone();
        }
        #endregion
        #endif

        /// <summary cref="Object.MemberwiseClone" />
        public new object MemberwiseClone()
        {
            temperature_readingCollection clone = new temperature_readingCollection(this.Count);

            for (int ii = 0; ii < this.Count; ii++)
            {
                clone.Add((temperature_reading)Utils.Clone(this[ii]));
            }

            return clone;
        }
    }
    #endregion
    #endif
    #endregion
}